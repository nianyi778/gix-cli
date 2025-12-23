import { Command } from 'commander'
import { execSync } from 'child_process'
import inquirer from 'inquirer'

const resetCommand = new Command('reset')
  .description('Discard all local unpushed commits (soft reset to remote)')
  .action(async () => {
    try {
      const branch = execSync('git rev-parse --abbrev-ref HEAD').toString().trim()
      const remote = execSync(`git config branch.${branch}.remote`).toString().trim()
      const remoteRef = `${remote}/${branch}`

      const confirm = await inquirer.prompt([
        {
          type: 'confirm',
          name: 'proceed',
          message: `⚠️  This will remove all local commits not pushed to ${remoteRef}. Proceed?`,
          default: false,
        },
      ])

      if (!confirm.proceed) {
        console.log('❌ Cancelled.')
        return
      }

      console.log(`\n🧨 Running: git reset --soft ${remoteRef}`)
      execSync(`git reset --soft ${remoteRef}`, { stdio: 'inherit' })
      console.log('✅ Local commits have been discarded (soft reset)')
    } catch (err) {
      console.error('❌ Failed to reset:', err)
    }
  })

export default resetCommand
