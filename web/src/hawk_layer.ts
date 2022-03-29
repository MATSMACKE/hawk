
import init, {Runner} from '@/hawk-wasm/pkg/hawk_wasm'

await init()

let code_runner = Runner.new()

let repl_runner = Runner.new()

export function run_code(code: string, in_repl: boolean) {
    if (in_repl) {
        repl_runner.run(code, true)
    } else {
        code_runner.run(code, false)
        code_runner.clear()
    }
}