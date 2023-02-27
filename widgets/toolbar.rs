// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{colours::RGBA, shapes, Shape, Size, Point},
    Align, DebugWidget, ToAny, Widget, widgets,
};

use super::{Button, Label};

/// Tool bar to be commonly displayed at the top of the application.
#[derive(Debug)]
pub struct ToolBar {
    /// The text displayed on the tool bar following `self.title_align`
    /// alignment.
    pub title: Option<Label>,
    /// Alignment rule for `self.title`, ignored if `self.title` is not defined.
    ///
    /// Must be either `Align::Left`, `Align::Center` or `Align::Right`.
    pub title_align: Align,
    /// Button performing some actions, to be displayed following
    /// `self.actions_align`.
    pub actions: Vec<Button>,
    /// Alignment rule for `self.actions`, ignored if `self.actions` is empty.
    ///
    /// Must be either `Align::Left`, `Align::Center` or `Align::Right`.
    pub actions_align: Align,
    /// The tool bar colour.
    pub colour: RGBA,
}

widgets::dynamic_widget!(ToolBar);

impl Widget for ToolBar {
    fn shape(&self, position: Option<Point>, size: Option<Size>) -> Shape {
        assert_eq!(position, None);
        assert!(size != None);

        // No need to define the position because it's located by default at 
        // [0, 0] as wanted.
        shapes::Builder::new()
            .rectangle(size.unwrap(), None)
            .fill(self.colour)
            .finish()
    }
}

impl Default for ToolBar {
    fn default() -> Self {
        ToolBar {
            title: None,
            title_align: Align::Center,
            actions: vec![],
            actions_align: Align::Right,
            colour: RGBA::default(),
        }
    }
}

impl ToolBar {
    pub fn new(
        title: Label,
        title_align: Align,
        actions: Vec<Button>,
        actions_align: Align,
        colour: RGBA,
    ) -> Self {
        Self {
            title: Some(title),
            title_align,
            actions,
            actions_align,
            colour,
        }
    }

    pub fn simple(actions: Vec<Button>, colour: RGBA) -> Self {
        Self {
            actions,
            colour,
            ..Self::default()
        }
    }

    pub fn titled(title: Label, actions: Vec<Button>) -> Self {
        Self {
            title: Some(title),
            actions,
            ..Self::default()
        }
    }

    pub fn coloured(actions: Vec<Button>, colour: RGBA) -> Self {
        Self {
            actions,
            colour,
            ..Self::default()
        }
    }
}
