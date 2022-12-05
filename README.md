# loggs

Simple opinionated logger for Windows and Linux applications

### Example

```rs

use loggs::Logger;

fn main() {
  let logger = Logger::new_default_location("testapp");

  // Add a log `Hello logs!` inside of a collection of logs named `main_app`
  logger.log("main_app", "Hello logs!");

  // Save logs explicitly
  logger.save_logs();

  // This will make it so if your app panics, the logs will be saved
  logger.save_on_panic();
}

```

The above code will create the following file structure:

on Windows - %AppData%/\[app_name\]/Logs/\[current_time\]/main_app.log.txt<br>
on Linux - /var/log/\[app_name\]/\[current_time\]/main_app.log.txt
