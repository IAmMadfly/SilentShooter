use amethyst::{
    ecs::prelude::Entity,
    input,
    prelude::*,
    ui::{UiCreator, UiFinder, UiEvent, UiEventType},
    winit::VirtualKeyCode
};

use crate::systems::game_time::PlayStateEnum;

pub struct WelcomeState {
    game_loader:    Option<()>,
    ui_handle:      Option<Entity>,
    start_butt:     Option<Entity>
}

impl Default for WelcomeState {
    fn default() -> Self {
        WelcomeState {
            game_loader:    Some(()),
            ui_handle:      None,
            start_butt:     None
        }
    }
}

impl SimpleState for WelcomeState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        
        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
        
        //game::load_game_map(world);
        if let Some(_loader) = &self.game_loader {
            // loader.borrow_mut().load_game_map(world);
            // loader.borrow_mut().init_camera(world);
        } else {
            panic!("Code should never get here");
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.start_butt.is_none() {
            state_data.world.exec(|ui_finder: UiFinder<'_>| {
                self.start_butt = ui_finder.find("start")
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self, 
        _data: StateData<'_, GameData<'_, '_>>, 
        event: StateEvent
    ) -> SimpleTrans {
        let world = _data.world;

        match event {
            StateEvent::Window(window_event) => {
                if input::is_close_requested(&window_event) {
                    Trans::Quit
                } else if input::is_key_down(&window_event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target
            }) => {
                if Some(target) == self.start_butt {
                    println!("Starting game!!");

                    let mut game_state = world.write_resource::<PlayStateEnum>();
                    *game_state = PlayStateEnum::InGame;
                    // return Trans::Switch(
                    //     Box::new(
                    //         self.game_loader.take().expect("Failed to get Game state!")
                    //     )
                    // )

                }
                Trans::None
            }
            _ => Trans::None
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {

        if let Some(root_entity) = self.ui_handle {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to delete Start screen");
        }

        self.ui_handle =    None;
        self.start_butt =   None;
    }
}