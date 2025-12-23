import { Command } from 'commander';
import mergeCommand from '../commands/merge';
import squashCommand from '../commands/squash';
import doctorCommand from '../commands/doctor';
import resetCommand from '../commands/reset';
import pkg from '../../package.json';

const program = new Command();

program
  .name('gix')
  .description('Gix: A Git extension CLI tool')
  .version(pkg.version);

program.addCommand(mergeCommand);
program.addCommand(squashCommand);
program.addCommand(doctorCommand);
program.addCommand(resetCommand);

program.parse();
