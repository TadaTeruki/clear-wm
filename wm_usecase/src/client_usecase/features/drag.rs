use std::error::Error;

use log::warn;
use wm_model::entity::{
    client::WindowType,
    cursor::DragMode,
    geometry::{Geometry, WmMoveResizeMask},
};
use x11rb::protocol::xproto::Window;

use crate::client_usecase::types::WmClientUseCase;

impl<'a> WmClientUseCase<'a> {
    pub fn usecase_start_to_drag_client(
        &mut self,
        window: Window,
        relative_x: i32,
        relative_y: i32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.collection_manager.query_from_window(window)? {
            if window != client.frame {
                warn!("invalid window type to drag");
                return Ok(());
            }
            let frame_geom_option = self
                .client_manager
                .get_geometry(client, WindowType::Frame)?;

            if let Some(frame_geom) = frame_geom_option {
                self.cursor_manager
                    .start_to_drag_client(client.app, relative_x, relative_y, frame_geom);
            }
        } else {
            warn!("client not found");
        }

        Ok(())
    }

    pub fn usecase_release_dragging_client(&mut self) -> Result<(), Box<dyn Error>> {
        self.cursor_manager.release_dragging_client();
        Ok(())
    }

    pub fn usecase_drag_client(
        &mut self,
        cursor_x: i32,
        cursor_y: i32,
    ) -> Result<(), Box<dyn Error>> {
        let drag_info = {
            if let Some(drag_info_) = self.cursor_manager.get_drag_info() {
                drag_info_
            } else {
                return Ok(());
            }
        };
        let (app, relative_x, relative_y, first_frame_geom, drag_mode) = (
            drag_info.app,
            drag_info.relative_x,
            drag_info.relative_y,
            drag_info.first_frame_geom,
            drag_info.drag_mode,
        );

        let client = match self.collection_manager.query_from_window(app)? {
            Some(client_) => client_,
            None => {
                return Ok(());
            }
        };

        let frame_geom_option = self
            .client_manager
            .get_geometry(client, WindowType::Frame)?;

        let current_frame_geom = {
            if let Some(current_frame_geom_) = frame_geom_option {
                current_frame_geom_
            } else {
                warn!("cannot query a geometry data for client");
                return Ok(());
            }
        };

        let (move_x, move_y) = (cursor_x - relative_x, cursor_y - relative_y);

        let (l_expand_width, t_expand_height) = (
            first_frame_geom.width - move_x + first_frame_geom.x,
            first_frame_geom.height - move_y + first_frame_geom.y,
        );

        let (r_expand_width, b_expand_height) = (
            first_frame_geom.width + move_x - first_frame_geom.x,
            first_frame_geom.height + move_y - first_frame_geom.y,
        );

        let mut need_redraw = true;

        self.server_manager.grab()?;

        match drag_mode {
            DragMode::Move => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: move_x,
                        y: move_y,
                        width: 0,
                        height: 0,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::Move,
                )?;
                need_redraw = false;
            }
            DragMode::ResizeTop => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: current_frame_geom.x,
                        y: move_y,
                        width: current_frame_geom.width,
                        height: t_expand_height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::MoveResize,
                )?;
            }
            DragMode::ResizeBottom => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: 0,
                        y: 0,
                        width: l_expand_width,
                        height: current_frame_geom.height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::Resize,
                )?;
            }
            DragMode::ResizeLeft => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: move_x,
                        y: current_frame_geom.y,
                        width: l_expand_width,
                        height: current_frame_geom.height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::MoveResize,
                )?;
            }
            DragMode::ResizeRight => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: 0,
                        y: 0,
                        width: r_expand_width,
                        height: current_frame_geom.height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::Resize,
                )?;
            }
            DragMode::ResizeTL => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: move_x,
                        y: move_y,
                        width: l_expand_width,
                        height: t_expand_height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::MoveResize,
                )?;
            }
            DragMode::ResizeTR => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: current_frame_geom.x,
                        y: move_y,
                        width: r_expand_width,
                        height: t_expand_height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::MoveResize,
                )?;
            }
            DragMode::ResizeBL => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: move_x,
                        y: current_frame_geom.y,
                        width: l_expand_width,
                        height: b_expand_height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::MoveResize,
                )?;
            }
            DragMode::ResizeBR => {
                self.client_manager.move_resize(
                    client,
                    Geometry {
                        x: 0,
                        y: 0,
                        width: r_expand_width,
                        height: b_expand_height,
                    },
                    WindowType::Frame,
                    WmMoveResizeMask::Resize,
                )?;
            }
        }

        if need_redraw {
            self.server_manager.sync()?;
            self.client_manager
                .draw_frame(client, self.property_manager.client_title(client)?)?;
        }

        self.server_manager.ungrab()?;

        Ok(())
    }
}
