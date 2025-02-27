const { spawn } = require('child_process');
const path = require('path');

const urlRegex = /(https:\/\/profiler\.firefox\.com[^\s]+)/;

function runScript() {
  // Get the path to run.sh relative to this script
  const shellScriptPath = 'run.sh';
  
  // Spawn the shell script process
  const shellProcess = spawn('sh', [shellScriptPath], {
      stdio: ['inherit', 'pipe', 'pipe']
  });

  // Listen for data on stdout
  shellProcess.stdout.on('data', (data) => {
    const output = data.toString();
    process.stdout.write(output);

    // Check if the output contains a URL and we haven't processed it yet
    const match = output.match(urlRegex);
    if (match) {
      const url = match[1];
      console.log('Found URL:', url);
      const urlData = url.split('127.0.0.1%3A')[1].split('%2F');
      const port = urlData[0];
      const id = urlData[1];
      console.log('id', id);
      console.log(`http://localhost:${port}/${id}`);

      // Wait for the shell script to complete before running follow-up commands
      const commands = [
        {
          cmd: 'gzip',
          args: ['-d', 'profile.json.gz'],
        },
        {
          cmd: 'node',
          args: [
            'bin/symbolicate/symbolicator-cli/symbolicator-cli.js',
            '--input',
            'profile.json',
            '--output',
            'profile-symbolicated.json',
            '--server',
            `http://localhost:${port}/${id}`
          ],
        },
      ];

      function executeSequential(commands, index = 0) {
        if (index >= commands.length) return;

        const command = commands[index];
        const nextCommand = spawn(command.cmd, command.args, {
          stdio: 'inherit'
        });

        nextCommand.on('error', (error) => {
          console.error('Failed to execute command:', error);
          process.exit(1);
        });

        nextCommand.on('close', (code) => {
          if (code === 0) {
            // Run next command only if current command succeeded
            executeSequential(commands, index + 1);
          } else {
            console.error(`Command failed with exit code ${code}`);
            process.exit(code);
          }
        });
      }

      executeSequential(commands);
    }
  });

  

  // Handle errors
  shellProcess.stderr.on('data', (data) => {
      process.stderr.write(`${data}`);
  });

  shellProcess.on('error', (error) => {
      console.error('Failed to start shell script:', error);
      process.exit(1);
  });
}

runScript();