match trimmed {
            "start" => {
                writeln!(handle, "Starting the timer")?;
                c.start()
                    .map(|res| writeln!(handle, "The response is {:?}", res))
                    .expect("Starting Error!")?;
            }
            "pause" => {
                writeln!(handle, "Pausing the thread")?;
            }
            "current" => {
                let current = p.current();
                match current {
                    Some(task) => writeln!(handle, "{}", format_task(task, default_width))?,
                    None => writeln!(handle, "There is no current task!")?,
                }
            }
            "complete" => match p.complete_next() {
                Some(task) => writeln!(
                    handle,
                    "Just completed: {}",
                    format_task(task, default_width)
                )?,
                None => writeln!(handle, "No more tasks to complete!")?,
            },
            "pop" => match p.tasks.pop_front() {
                Some(task) => {
                    writeln!(handle, "Just popped: {}", format_task(&task, default_width))?
                }
                None => writeln!(handle, "No more tasks to pop!")?,
            },
            "tasks" => {
                format_tasks(p).iter().for_each(|line| println!("{}", line));
            }
            "timer" => {
                let result = c.info();
                match result {
                    Ok(res) => match res {
                        Response::Ticking(duration) => writeln!(
                            handle,
                            "The timer has {} seconds remaining",
                            duration.as_secs()
                        )?,

                        Response::Resetting => writeln!(handle, "The timer is resetting")?,

                        _ => writeln!(handle, "No tick currently!")?,
                    },
                    _ => writeln!(handle, "No tick!")?,
                }
            }
            _ => writeln!(handle, "'{}' is not a valid command!", trimmed)?,
        }