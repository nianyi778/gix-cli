/// gix - Git Extension CLI (TypeScript version)

import { Command } from 'commander';
import inquirer from 'inquirer';
import { execSync } from 'child_process';

const mergeCommand = new Command('merge')
  .description('Merge multiple git commits')
  .option('-f, --from <hash>', 'Start commit hash (required)')
  .option('-t, --to <hash>', 'End commit hash (default is HEAD)')
  .option('-m, --msg <message>', 'New commit message')
  .action(async (options) => {
    const questions = [];

    if (!options.from) {
      questions.push({
        type: 'input',
        name: 'from',
        message: 'Enter the start commit hash:',
        validate: (input: string) => !!input || 'Start commit hash is required.',
      });
    }

    if (!options.to) {
      questions.push({
        type: 'input',
        name: 'to',
        message: 'Enter the end commit hash (leave blank for HEAD):',
      });
    }

    if (!options.msg) {
      questions.push({
        type: 'input',
        name: 'msg',
        message: 'Enter the new commit message:',
        validate: (input: string) => !!input || 'Commit message cannot be empty.',
      });
    }

    const answers = await inquirer.prompt(questions);
    const from = options.from || answers.from;
    const to = options.to || answers.to || 'HEAD';
    const msg = options.msg || answers.msg;

    const rootCommits = execSync(`git rev-list --max-parents=0 HEAD`).toString().trim().split('\n');
    if (rootCommits.includes(from)) {
      console.error('❌ Cannot merge from the first (root) commit — it has no parent.\n👉 Consider using `git rebase --root` instead.');
      return;
    }

    // 检查工作区是否干净
    const status = execSync('git status --porcelain').toString().trim();
    if (status) {
      console.error('❌ Your working directory is not clean. Please commit, stash, or reset changes before merging.');
      return;
    }

    const resetCmd = `git reset --soft ${from}^`;
    const commitCmd = `git commit --edit -m "${msg}" --no-verify`;

    console.log(`\n🔧 Executing:\n${resetCmd} && ${commitCmd}\n`);

    try {
      execSync(resetCmd, { stdio: 'inherit' });
      console.log('✅ Reset successful');
    } catch (err) {
      console.error('❌ Reset failed:', err);
      return;
    }

    try {
      execSync(commitCmd, { stdio: 'inherit' });
      console.log('✅ Commit successful');
    } catch (err) {
      console.error('❌ Commit failed:', err);
      return;
    }

    try {
      const { pushConfirm } = await inquirer.prompt([
        {
          type: 'list',
          name: 'pushConfirm',
          message: 'Do you want to force push automatically?',
          choices: [
            { name: '✅ Yes (default)', value: 'yes' },
            { name: '❌ No, I will push manually', value: 'no' },
          ],
          default: 'yes'
        }
      ]);

      if (pushConfirm === 'yes') {
        try {
          const currentBranch = execSync('git symbolic-ref --short HEAD').toString().trim();
          let hasUpstream = false;
          try {
            execSync(`git rev-parse --abbrev-ref ${currentBranch}@{u}`, { stdio: 'ignore' });
            hasUpstream = true;
          } catch {
            hasUpstream = false;
          }

          if (!hasUpstream) {
            console.log(`\n🚀 No upstream detected. Executing: git push --set-upstream origin ${currentBranch}\n`);
            execSync(`git push --set-upstream origin ${currentBranch}`, { stdio: 'inherit' });
            console.log('✅ Push & upstream set successfully');
          } else {
            console.log('\n🚀 Executing git push --force-with-lease\n');
            execSync('git push --force-with-lease', { stdio: 'inherit' });
            console.log('✅ Force push successful');
          }
        } catch (pushErr) {
          console.error('❌ Push failed:', pushErr);
        }
      } else {
        console.log('\n⚠️ Please push manually using git push\n');
      }
    } catch (err) {
      console.error('❌ Push failed:', err);
    }
  });

export default mergeCommand;