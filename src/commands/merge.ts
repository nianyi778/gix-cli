/// gix - Git Extension CLI (TypeScript version)

import { Command } from 'commander';
import inquirer from 'inquirer';
import { execSync } from 'child_process';

const mergeCommand = new Command('merge')
  .description('合并多个 git commit')
  .option('-f, --from <hash>', '起始 commit hash（必填）')
  .option('-t, --to <hash>', '结束 commit hash（默认 HEAD）')
  .option('-m, --msg <message>', '新的 commit message')
  .action(async (options) => {
    const questions = [];

    if (!options.from) {
      questions.push({
        type: 'input',
        name: 'from',
        message: '请输入起始 commit（hash）:',
        validate: (input: string) => !!input || '起始 commit 不能为空',
      });
    }

    if (!options.to) {
      questions.push({
        type: 'input',
        name: 'to',
        message: '请输入结束 commit（默认 HEAD 可留空）:',
      });
    }

    if (!options.msg) {
      questions.push({
        type: 'input',
        name: 'msg',
        message: '请输入新的 commit message:',
        validate: (input: string) => !!input || 'commit message 不能为空',
      });
    }

    const answers = await inquirer.prompt(questions);
    const from = options.from || answers.from;
    const to = options.to || answers.to || 'HEAD';
    const msg = options.msg || answers.msg;

    const resetCmd = `git reset --soft ${from}^`;
    const commitCmd = `git commit --edit -m "${msg}" --no-verify`;

    console.log(`\n🔧 准备执行：\n${resetCmd} && ${commitCmd}\n`);

    try {
      execSync(resetCmd, { stdio: 'inherit' });
      console.log('✅ reset 成功');
    } catch (err) {
      console.error('❌ reset 执行失败：', err);
      return;
    }

    try {
      execSync(commitCmd, { stdio: 'inherit' });
      console.log('✅ commit 成功');
    } catch (err) {
      console.error('❌ commit 执行失败：', err);
      return;
    }

    try {
      const { pushConfirm } = await inquirer.prompt([
        {
          type: 'list',
          name: 'pushConfirm',
          message: '是否执行自动强推？',
          choices: [
            { name: '✅ 是（默认）', value: 'yes' },
            { name: '❌ 否，我手动推送', value: 'no' },
          ],
          default: 'yes'
        }
      ]);

      if (pushConfirm === 'yes') {
        console.log('\n🚀 正在执行 git push --force-with-lease\n');
        execSync('git push --force-with-lease', { stdio: 'inherit' });
        console.log('✅ 强推成功');
      } else {
        console.log('\n⚠️ 请手动执行 git push\n');
      }
    } catch (err) {
      console.error('❌ push 执行失败：', err);
    }
  });

export default mergeCommand;