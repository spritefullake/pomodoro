use super::{
    pomodoro::Pomodoro,
    task::Task,
};

impl std::fmt::Display for Task{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let modifier = match self.is_complete() {
            true  => "",
            false => "not",
        };
        write!(f, "{} is {} completed", self.title, modifier)
    }
}
impl std::fmt::Display for Pomodoro{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let break_indication = match self.is_on_break() {
            true => "On Break\n",
            false => "",
        };
        write!(f, "{}{}", break_indication, format_tasks(self).join("\n"))
    }
}

pub fn format_task(task: &Task, width: usize) -> String {
    let mut completion = " ";

    if task.is_complete() {
        completion = "✓";
    };

    format!("{1:0$} | [{2}]", width, task.title, completion)
}
pub fn format_tasks(p: &Pomodoro) -> Vec<String> {
    let task_width = p
        .tasks
        .iter()
        .map(|task| task.title.len())
        .max()
        .unwrap_or(0);
    let current_marker = "<<";
    let delimiter = "\n";
    let header = format!(
        "{1:^0$} | {2}{3}",
        task_width, "Tasks", "Complete?", delimiter
    );

    let tasks = p
        .tasks
        .iter()
        .map(|task| {
            let formatted = format_task(task, task_width);
            if let Some(current) = p.current() {
                //check pointer equality since == not implemented on Tasks
                if current as *const _ == task as *const _ {
                    format!("{}{}{}", formatted, current_marker, delimiter)
                } else {
                    format!("{}{}", formatted, delimiter)
                }
            } else {
                format!("{}{}", formatted, delimiter)
            }
        })
        .collect();

    vec![header, tasks]
}