// This file is part of the uutils procps package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use crate::selected_fields;
use crate::tui::input::InputMode;
use std::time::Duration;

pub(crate) struct TuiStat {
    pub input_mode: InputMode,
    pub input_label: String,
    pub input_value: String,
    pub input_message: Option<String>, // Info or error
    pub selected_process: Option<u32>,

    pub show_load_avg: bool,
    pub cpu_graph_mode: CpuGraphMode,
    pub cpu_value_mode: CpuValueMode,
    pub memory_graph_mode: MemoryGraphMode,
    pub cpu_column: u16,
    pub list_offset: usize,
    pub horizontal_offset: usize,
    pub max_list_display: usize,
    pub colorful: bool,
    pub full_command_line: bool,
    pub delay: Duration,
    pub sorter: String,
    pub sort_by_pid: bool,
    pub highlight_sorted: bool,
    pub highlight_bold: bool,
    pub show_coordinates: bool,
    pub show_zeros: bool,
    pub irix_mode: bool,
    pub width_increment: Option<usize>, // None means auto
    pub time_scale: TimeScale,

    pub filter: Option<crate::Filter>,
}

impl TuiStat {
    pub fn new() -> Self {
        let fields = selected_fields();
        let filter = if fields.contains(&"%CPU".to_string()) {
            "%CPU".to_string()
        } else {
            fields[0].clone()
        };

        Self {
            input_mode: InputMode::Command,
            input_label: String::new(),
            input_value: String::new(),
            input_message: None,
            selected_process: None,

            show_load_avg: true,
            cpu_graph_mode: CpuGraphMode::default(),
            cpu_value_mode: CpuValueMode::default(),
            memory_graph_mode: MemoryGraphMode::default(),
            cpu_column: 2,
            list_offset: 0,
            horizontal_offset: 0,
            max_list_display: 0, // unlimited
            colorful: true,
            full_command_line: true,
            delay: Duration::from_millis(1500), // 1.5s
            sorter: filter,
            sort_by_pid: false,
            highlight_sorted: false,
            highlight_bold: false,
            show_coordinates: false,
            show_zeros: true,
            irix_mode: true,
            width_increment: Some(0), // fixed
            time_scale: TimeScale::default(),

            filter: None,
        }
    }

    pub fn reset_input(&mut self) {
        self.input_mode = InputMode::Command;
        self.input_label.clear();
        self.input_value.clear();
        self.input_message = None;
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum CpuGraphMode {
    #[default]
    Block,
    Bar,
    Sum,
    Hide,
}

impl CpuGraphMode {
    pub fn next(&self) -> CpuGraphMode {
        match self {
            CpuGraphMode::Block => CpuGraphMode::Hide,
            CpuGraphMode::Hide => CpuGraphMode::Sum,
            CpuGraphMode::Sum => CpuGraphMode::Bar,
            CpuGraphMode::Bar => CpuGraphMode::Block,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum CpuValueMode {
    #[default]
    PerCore,
    Numa,
    NumaNode(usize),
    Sum,
}

impl CpuValueMode {
    pub fn next(&self) -> CpuValueMode {
        match self {
            CpuValueMode::PerCore => CpuValueMode::Sum,
            CpuValueMode::Sum => CpuValueMode::PerCore,
            CpuValueMode::Numa => CpuValueMode::Sum,
            CpuValueMode::NumaNode(_) => CpuValueMode::PerCore,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum MemoryGraphMode {
    #[default]
    Block,
    Bar,
    Sum,
    Hide,
}

impl MemoryGraphMode {
    pub fn next(&self) -> MemoryGraphMode {
        match self {
            MemoryGraphMode::Block => MemoryGraphMode::Hide,
            MemoryGraphMode::Hide => MemoryGraphMode::Sum,
            MemoryGraphMode::Sum => MemoryGraphMode::Bar,
            MemoryGraphMode::Bar => MemoryGraphMode::Block,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum TimeScale {
    #[default]
    MinSecondCent, // 00:00.00
    MinSecond, // 00:00
    HourMin,   // 0,00
    DayHour,   // 0d+0h
    Day,       // 0d
    WeekDay,   // 0w+0d
    Week,      // 0w
}

impl TimeScale {
    pub fn next(&self) -> TimeScale {
        match self {
            TimeScale::MinSecondCent => TimeScale::MinSecond,
            TimeScale::MinSecond => TimeScale::HourMin,
            TimeScale::HourMin => TimeScale::DayHour,
            TimeScale::DayHour => TimeScale::Day,
            TimeScale::Day => TimeScale::WeekDay,
            TimeScale::WeekDay => TimeScale::Week,
            TimeScale::Week => TimeScale::MinSecondCent,
        }
    }
}
