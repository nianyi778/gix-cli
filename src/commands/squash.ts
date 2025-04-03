import { Command } from 'commander'
import { execSync } from 'child_process'

const squashCommand = new Command('squash')
  .description('Squash recent commits into one')
  .option('-n, --number <count>', 'Number of commits to squash (default 2)', '2')
  .option('--all', 'Squash all commits from the beginning')
  .action((options) => {
    // 检查工作区是否干净
    const status = execSync('git status --porcelain').toString().trim()
    if (status) {
      console.error('❌ Your working directory is not clean. Please commit, stash, or reset changes before squashing.')
      return
    }

    if (options.all) {
      console.log('🧨 Running: git rebase -i --root')
      execSync('git rebase -i --root', { stdio: 'inherit' })
    } else {
      const count = Number(options.number)
      console.log(`🧨 Running: git rebase -i HEAD~${count}`)
      execSync(`git rebase -i HEAD~${count}`, { stdio: 'inherit' })
    }
  })

export default squashCommand