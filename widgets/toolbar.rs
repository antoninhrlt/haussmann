// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{Widget, graphics::{Size, Shape, shapes, colours::RGBA}, Align, ToAny, DebugWidget};

use super::{Label, Button};

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

crate::dynamic_widget!(ToolBar);

impl Widget for ToolBar {
    fn shape(&self, size: Option<Size>) -> Shape {
        assert!(size != None);

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
    pub fn new(title: Label, title_align: Align, actions: Vec<Button>, actions_align: Align, colour: RGBA) -> Self {
        Self {
            title: Some(title),
            title_align,
            actions,
            actions_align,
            colour
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
