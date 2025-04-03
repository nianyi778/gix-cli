import { Command } from 'commander';
import mergeCommand from '../commands/merge';

const program = new Command();

program
  .name('gix')
  .description('Gix: A Git extension CLI tool')
  .version('1.0.0');

program.addCommand(mergeCommand);

program.parse();
