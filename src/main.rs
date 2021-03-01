
use amethyst::{
    self, 
    Application, 
    GameDataBuilder, 
    core::transform::TransformBundle, 
    input::{
        StringBindings,
        InputBundle
    }, renderer::{
        RenderingBundle,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend
    }, 
    ui::{RenderUi, UiBundle}
};


mod systems;
mod states;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = amethyst::utils::application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with(
            amethyst::utils::ortho_camera::CameraOrthoSystem::default(),
            "ortho_camera_system",
            &[]
        )
        .with_system_desc(
            systems::game_time::GameTimeSystemDesc::default(), 
            "game_time", 
            &[]
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .expect("Failed to attach RenderToWindowBundle")
                        .with_clear([0.005, 0.005, 0.005, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(InputBundle::<StringBindings>::new()).expect("Failed to attach InputBundle")
        .with_bundle(TransformBundle::new()).expect("Failed to attach TransformBundle")
        .with_bundle(UiBundle::<StringBindings>::new()).expect("Failed to attach UiBundle");

        let mut game = Application::new(
            assets_dir, 
            states::welcome::WelcomeState::default(), 
            game_data
        ).expect("Failed to create new Game Application");

        game.run();

        Ok(())
}
