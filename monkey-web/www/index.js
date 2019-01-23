import { Interpreter } from 'monkey-web';

const PROMPT = '>>>';

const interpreter = Interpreter.new();

const inputField = document.getElementById('input');

const stdoutContainer = document.getElementById('stdout');

inputField.addEventListener('keyup', element => {
  if (event.key === 'Enter') {
    const input = element.target.value;

    appendOutput(`${PROMPT} ${input}`);

    const output = interpreter.evaluate(input);

    appendOutput(output);

    element.target.value = '';
  }
});

const appendOutput = text => {
  const div = document.createElement('div');
  div.textContent = text;

  stdoutContainer.appendChild(div);
};
