const { spawn } = require('child_process');
const fs = require('fs');

const urlRegex = /(https:\/\/profiler\.firefox\.com[^\s]+)/;

function runScript() {
  // Script must be run from the root of the repository
  if (!fs.existsSync('rust-toolchain.toml')) {
    console.error('Error: This script must be run from the repository root');
    process.exit(1);
  }

  // Get the shell script path from command line arguments, or use default
  if (process.argv.length < 3) {
    console.error('Error: Please provide the path to the shell script (from repo root) that runs `samply record` as the first argument');
    process.exit(1);
  }
  const shellScriptPath = process.argv[2];
  
  // Get the metric name from command line arguments, or use default `profile` name
  const metricName = process.argv[3] || "profile"; 
  
  // Spawn the shell script process
  const shellProcess = spawn('sh', [shellScriptPath], {
      stdio: ['inherit', 'pipe', 'pipe']
  });

  // Listen for data on stdout
  shellProcess.stdout.on('data', (data) => {
    const output = data.toString();
    process.stdout.write(output)

    // Check if the output contains a URL and we haven't processed it yet
    const match = output.match(urlRegex);
    if (match) {
      const url = match[1];
      const urlData = url.split('127.0.0.1%3A')[1].split('%2F');
      const port = urlData[0];
      const id = urlData[1];
      const serverUrl = `http://localhost:${port}/${id}`;
      console.log('server url:', serverUrl);

      // Wait for the shell script to complete before running follow-up commands
      const commands = [
        {
          cmd: 'gzip',
          args: ['-d', '-f', 'profile.json.gz'],
        },
        {
          cmd: 'mv',
          args: ['profile.json', `${metricName}.json`],
        },
        {
          cmd: 'ls',
          args: ['-lF'],
        },
        {
          cmd: 'sleep',
          args: ['5'],
        },
        {
          cmd: 'node',
          args: [
            'bin/symbolicate/symbolicator-cli/symbolicator-cli.js',
            '--input',
            `${metricName}.json`,
            '--output',
            `${metricName}-symbolicated.json`,
            '--server',
            serverUrl
          ],
        },
      ];

      function executeSequential(commands, index = 0) {
        if (index >= commands.length) return;

        const command = commands[index];
        const nextCommand = spawn(command.cmd, command.args, {
          stdio: ['inherit', 'pipe', 'pipe'],
        });

        nextCommand.stdout.on('data', (data) => {
          const output = data.toString();
          process.stdout.write(output);
          if (output.includes('Finished.')) {
            console.log('Exiting symbolicator.');
            process.exit(0);
          }
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