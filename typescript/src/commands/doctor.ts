import { Command } from 'commander'
import { execSync } from 'child_process'

const doctorCommand = new Command('doctor')
  .description('Check git working directory and environment')
  .action(() => {
    console.log('🩺 Running gix health check...\n')

    // Node.js 版本检查
    const nodeVersion = process.versions.node
    const [major] = nodeVersion.split('.')
    if (parseInt(major) < 18) {
      console.warn(`⚠️  Node.js version is ${nodeVersion}. Recommended version is >= 18.`)
    } else {
      console.log(`✅ Node.js version: ${nodeVersion}`)
    }

    // 检查 git 是否可用
    try {
      const version = execSync('git --version').toString().trim()
      console.log(`✅ Git installed: ${version}`)
    } catch (err) {
      console.error('❌ Git not found in PATH')
      return
    }

    // 检查是否在 git 仓库中
    try {
      execSync('git rev-parse --is-inside-work-tree', { stdio: 'ignore' })
      console.log('✅ Inside a Git repository')
    } catch (err) {
      console.error('❌ Not inside a Git repository')
      return
    }

    // 检查工作区是否干净
    const status = execSync('git status --porcelain').toString().trim()
    if (status) {
      console.log('⚠️  Working directory has uncommitted changes')
    } else {
      console.log('✅ Working directory is clean')
    }

    // 检查是否存在远程仓库
    try {
      const remotes = execSync('git remote').toString().trim()
      if (remotes) {
        console.log(`✅ Remote(s) configured: ${remotes}`)
      } else {
        console.log('⚠️  No Git remotes configured')
      }
    } catch (err) {
      console.log('⚠️  Failed to check remotes')
    }

    // 当前分支信息
    try {
      const branch = execSync('git branch --show-current').toString().trim()
      console.log(`📍 Current branch: ${branch}`)
    } catch {
      console.log('⚠️  Failed to get current branch')
    }

    console.log('\n🧩 Done.')
  })

export default doctorCommand