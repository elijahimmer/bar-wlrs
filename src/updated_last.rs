use crate::draw::*;
use crate::widget::Widget;

use anyhow::Result;
use chrono::{DateTime, TimeDelta, Utc};

pub struct UpdatedLast<'a> {
    name: Box<str>,
    time: DateTime<Utc>,
    text: TextBox<'a>,
    last_text_set: Box<str>,
}

impl UpdatedLast<'_> {
    pub fn builder() -> UpdatedLastBuilder {
        Default::default()
    }
}

impl Widget for UpdatedLast<'_> {
    fn name(&self) -> &str {
        &self.name
    }
    fn area(&self) -> Rect {
        self.text.area()
    }
    fn h_align(&self) -> Align {
        self.text.h_align()
    }
    fn v_align(&self) -> Align {
        self.text.v_align()
    }
    fn desired_height(&self) -> u32 {
        self.text.desired_height()
    }
    fn desired_width(&self, height: u32) -> u32 {
        height * MAX_LABEL_LEN * 2 / 3
    }
    fn resize(&mut self, area: Rect) {
        self.text.resize(area);
    }
    fn draw(&mut self, ctx: &mut DrawCtx) -> Result<()> {
        let now = Utc::now();

        let new_text = &label_from_time(now - self.time);
        if *new_text != *self.last_text_set {
            self.last_text_set = new_text.clone().into();
            self.text.set_text(new_text);
            self.text.draw(ctx)
        } else {
            Ok(())
        }
    }
}

const MAX_LABEL_LEN: u32 = "59 Minutes Ago".len() as u32;
fn label_from_time(delta_time: TimeDelta) -> String {
    if delta_time.num_seconds() < 0 {
        return "The Future?".to_string();
    }

    let days = delta_time.num_days();
    if days > 14 {
        return "UPDATE NOW!".to_string();
    }
    match days.cmp(&1) {
        core::cmp::Ordering::Equal => return "1 Day Ago".to_string(),
        core::cmp::Ordering::Greater => return format!("{days} Days Ago"),
        core::cmp::Ordering::Less => {}
    }

    let hours = delta_time.num_hours();
    match hours.cmp(&1) {
        core::cmp::Ordering::Equal => return "1 Hour Ago".to_string(),
        core::cmp::Ordering::Greater => return format!("{hours} Hours Ago"),
        core::cmp::Ordering::Less => {}
    }

    let minutes = delta_time.num_minutes();
    match minutes.cmp(&1) {
        core::cmp::Ordering::Equal => return "1 Minute Ago".to_string(),
        core::cmp::Ordering::Greater => return format!("{minutes} Minutes Ago"),
        core::cmp::Ordering::Less => {}
    }

    "Now".to_string()
}

#[derive(Clone, Debug, Default)]
pub struct UpdatedLastBuilder {
    time_stamp: i64,
    desired_height: Option<u32>,
    h_align: Align,
    v_align: Align,
    fg: Color,
    bg: Color,
}

impl UpdatedLastBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    crate::builder_fields! {
        i64, time_stamp;
        u32, desired_height;
        Align, v_align h_align;
        Color, fg bg;
    }

    pub fn build<'a>(&self, name: &str) -> UpdatedLast<'a> {
        log::info!(
            "'{name}' | new :: initializing with height: {}",
            self.desired_height.unwrap_or(u32::MAX)
        );
        let time = chrono::DateTime::from_timestamp(self.time_stamp, 0)
            .unwrap_or(chrono::DateTime::UNIX_EPOCH);

        let text = TextBox::builder()
            .v_align(self.v_align)
            .h_align(self.h_align)
            .fg(self.fg)
            .bg(self.bg)
            .text("Default Text")
            .desired_text_height(self.desired_height.unwrap_or(u32::MAX) * 20 / 23)
            .build(&(name.to_owned() + " Text"));

        UpdatedLast {
            name: name.into(),
            time,
            text,
            last_text_set: "Default Text".into(),
        }
    }
}