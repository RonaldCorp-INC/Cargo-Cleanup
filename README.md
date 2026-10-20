# Cargo-Cleanup
Cargo-Cleanup is a tool that will automatically clean all of the garbage files up for you in your main /Rust directory. Currently, 
I'm not sure how well this would work on Windows, as I have no Windows environment to test this tool on. **So for the time being, 
this tool is Linux exclusive!** 

# HOW-TO USE 
In order to use it, download the release, and place it in ~/.local/bin/ as you usually would, and make it executable.
After that, use the terminal to navigate to the folder containing all of your Rust projects, and then type "cargo-cleanup" 
in the CLI. This will automatically remove all the garbage that these Rust projects generate while working on your projects! 

# SAFETY DISCLAIMER 
This program *deletes all target directories*. So, you will obviously have to re-compile anything this program touches. Which
would be EVERY PROJECT IN YOUR PROJECTS DIRECTORY. Keep that in mind. 
