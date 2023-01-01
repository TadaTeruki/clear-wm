use std::error::Error;

use x11rb::protocol::xproto::Window;
use zym_model::entity::{client::WindowType, cursor::DragMode, geometry::Geometry};

use crate::client::usecase::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn start_to_drag_client(
        &mut self,
        window: Window,
        relative_x: i32,
        relative_y: i32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some((client_id, WindowType::Frame)) = self.client_manager.query(window) {
            let frame_geom_option = self
                .client_manager
                .get_geometry(client_id, WindowType::Frame)?;

            if let Some(frame_geom) = frame_geom_option {
                self.cursor_manager
                    .start_to_drag_client(client_id, relative_x, relative_y, frame_geom);
            }
        }
        Ok(())
    }

    pub fn release_dragging_client(&mut self) -> Result<(), Box<dyn Error>> {
        self.cursor_manager.release_dragging_client();
        Ok(())
    }

    pub fn drag_client(&mut self, cursor_x: i32, cursor_y: i32) -> Result<(), Box<dyn Error>> {
        if let Some(drag_info) = self.cursor_manager.get_drag_info() {
            let (client_id, relative_x, relative_y, first_frame_geom, drag_mode) = (
                drag_info.client_id,
                drag_info.relative_x,
                drag_info.relative_y,
                drag_info.first_frame_geom,
                drag_info.drag_mode,
            );

            let frame_geom_option = self
                .client_manager
                .get_geometry(client_id, WindowType::Frame)?;

            if let Some(current_frame_geom) = frame_geom_option {
                let (move_x, move_y) = (cursor_x - relative_x, cursor_y - relative_y);

                let (l_expand_width, t_expand_height) = (
                    first_frame_geom.width - move_x + first_frame_geom.x,
                    first_frame_geom.height - move_y + first_frame_geom.y,
                );

                let (r_expand_width, b_expand_height) = (
                    first_frame_geom.width + move_x - first_frame_geom.x,
                    first_frame_geom.height + move_y - first_frame_geom.y,
                );

                match drag_mode {
                    DragMode::Move => {
                        self.client_manager.move_to(
                            client_id,
                            move_x,
                            move_y,
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeTop => {
                        self.client_manager.move_resize(
                            client_id,
                            Geometry {
                                x: current_frame_geom.x,
                                y: move_y,
                                width: current_frame_geom.width,
                                height: t_expand_height,
                            },
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeBottom => {
                        self.client_manager.resize(
                            client_id,
                            current_frame_geom.width,
                            b_expand_height,
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeLeft => {
                        self.client_manager.move_resize(
                            client_id,
                            Geometry {
                                x: move_x,
                                y: current_frame_geom.y,
                                width: l_expand_width,
                                height: current_frame_geom.height,
                            },
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeRight => {
                        self.client_manager.resize(
                            client_id,
                            r_expand_width,
                            current_frame_geom.height,
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeTL => {
                        self.client_manager.move_resize(
                            client_id,
                            Geometry {
                                x: move_x,
                                y: move_y,
                                width: l_expand_width,
                                height: t_expand_height,
                            },
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeTR => {
                        self.client_manager.move_resize(
                            client_id,
                            Geometry {
                                x: current_frame_geom.x,
                                y: move_y,
                                width: r_expand_width,
                                height: t_expand_height,
                            },
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeBL => {
                        self.client_manager.move_resize(
                            client_id,
                            Geometry {
                                x: move_x,
                                y: current_frame_geom.y,
                                width: l_expand_width,
                                height: b_expand_height,
                            },
                            WindowType::Frame,
                        )?;
                    }
                    DragMode::ResizeBR => {
                        self.client_manager.resize(
                            client_id,
                            r_expand_width,
                            b_expand_height,
                            WindowType::Frame,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
