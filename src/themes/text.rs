// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::FontWeight;

/// Style for a text such as its size, font weight and spacing between the 
/// letters.
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    /// Size of the text.
    pub size: i32,
    /// Weight of text font.
    pub weight: FontWeight,
    /// Size between letters.
    pub spacing: f32,
}

/// Text styles for every text such as headings, paragraphs and code blocks...
#[derive(Debug, PartialEq)]
pub struct TextTheme {
    /// Font style for code blocks.
    pub code: TextStyle,
    /// Font style for headings level 1.
    pub heading1: TextStyle,
    /// Font style for headings level 2.
    pub heading2: TextStyle,
    /// Font style for headings level 3.
    pub heading3: TextStyle,
    /// Font style for headings level 4.
    pub heading4: TextStyle,
    /// Font style for headings level 5.
    pub heading5: TextStyle,
    /// Font style for headings level 6.
    pub heading6: TextStyle,
    /// Font style for paragraphs level 1.
    pub paragraph1: TextStyle,
    /// Font style for paragraphs level 2.
    pub paragraph2: TextStyle,
    /// Font style for paragraphs level 3.
    pub paragraph3: TextStyle,
}

