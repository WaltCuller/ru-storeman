pub fn help() -> &'static str {
    "
Tasks:
  storeman check                      # Show entries in Procfile
  storeman help [TASK]                # Show this help
  storeman export [FORMAT] [LOCATION] # Export the apps to another process
                                       (upstart)
  storeman run COMMAND [PROCESS...]   # Run a command
                                       start
                                       stop
                                       stop-all
                                       restart
                                       restart-all
                                       list
                                       status
  storeman start [PROCESS]            # Start the application
  storeman version                    # Display ru-storeman version
    "
}