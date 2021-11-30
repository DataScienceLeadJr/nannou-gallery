use nannou::prelude::*;

pub trait Artwork<T> {
    fn setup(&self);
    fn model(app: &App) -> T;
    fn update(app: &App, model: &mut T, _update: Update);
    fn view(app: &App, model: &T, frame: Frame);
}

pub trait InteractiveArtwork<T>: Artwork<T> {
    fn key_pressed(app: &App, model: &mut T, key: Key);
}

pub trait GUIArtwork<T>: Artwork<T> {
    fn ui_event(_app: &App, _model: &mut T, _event: WindowEvent);
    fn ui_view(_app: &App, _model: &T, _frame: Frame);
}