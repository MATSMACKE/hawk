
import init, {run} from '@/hawk-wasm/pkg/hawk_wasm'

await init()

export function run_code(code: string) {
    run(code, true)
}