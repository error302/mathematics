/// <reference lib="webworker" />
import init, { generate_exercise, check_answer, exercise_coaching, study_check } from '../../../../packages/axiom-wasm-web/axiom_wasm';

const ready = init();
self.onmessage = async (event: MessageEvent) => {
  const { id, operation, args } = event.data;
  try {
    await ready;
    let result: string;
    if (operation === 'generate') {
      result = generate_exercise(args[0], args[1], args[2], args[3], args[4]);
    } else if (operation === 'check') {
      result = check_answer(args[0], args[1]);
    } else if (operation === 'coach') {
      result = exercise_coaching(args[0]);
    } else if (operation === 'study-check') {
      result = study_check(args[0], args[1], args[2], args[3], args[4]);
    } else {
      throw new Error('Unsupported mathematical operation.');
    }
    self.postMessage({ id, result: JSON.parse(result) });
  } catch (error) {
    self.postMessage({ id, error: String(error) });
  }
};
