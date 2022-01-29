use crate::utils::{Message, PlayerInput};
use rusty_games_library::one_to_many::MiniClient;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ClientGame {
    mini_client: MiniClient,
}

impl ClientGame {
    fn start_as_gamer(&mut self) {
        let mini_client = self.mini_client.clone();
        let on_open_callback = move |_| {
            let mini_client = mini_client.clone();
            let g = Closure::wrap(Box::new(move || {
                crate::check_timer_from_js();

                // on each frame, send input to host
                let message = serde_json::to_string::<PlayerInput>(
                    &crate::get_player_input_from_js().into_serde().unwrap(),
                )
                .unwrap();
                // allow some messages to fail
                let _ = mini_client.send_message_to_host(&message);

                crate::draw_from_js();
            }) as Box<dyn FnMut()>);
            crate::utils::set_interval_with_callback(&g);
            g.forget();
        };

        let on_message_callback = move |_, message: String| {
            let message = serde_json::from_str::<Message>(&message).unwrap();

            // match message {
            //     Message::GameState {
            //         red_x,
            //         red_y,
            //         blue_x,
            //         blue_y,
            //         ball_x,
            //         ball_y,
            //     } => {
            //         {
            //             let red_body = &mut rigid_body_set_clone.borrow_mut()[red_handle];
            //             red_body.set_position(Isometry::new(vector![red_x, red_y], 0.0), false);
            //         }
            //         {
            //             let blue_body = &mut rigid_body_set_clone.borrow_mut()[blue_handle];
            //             blue_body.set_position(Isometry::new(vector![blue_x, blue_y], 0.0), false);
            //         }
            //         {
            //             let ball_body = &mut rigid_body_set_clone.borrow_mut()[ball_handle];
            //             ball_body.set_position(Isometry::new(vector![ball_x, ball_y], 0.0), false);
            //         }
            //     }
            //     Message::GoalScored {
            //         did_red_score: did_red_scored,
            //     } => {
            //         if did_red_scored {
            //             arbiter_clone.borrow_mut().set_red_scored();
            //         } else {
            //             arbiter_clone.borrow_mut().set_blue_scored();
            //         }
            //         arbiter_clone.borrow_mut().reset_timer = RESET_TIME;
            //     }
            // };
        };

        self.mini_client
            .start(on_open_callback, on_message_callback)
            .expect("network manager failed to start");
    }
}
