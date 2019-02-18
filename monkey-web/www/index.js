import { Interpreter } from 'monkey-web';

const PROMPT = '>>>';

const interpreter = Interpreter.new();

const inputField = document.getElementById('input');

const stdoutContainer = document.getElementById('stdout');

inputField.addEventListener('keyup', element => {
  if (event.key === 'Enter') {
    const input = element.target.value;
    appendOutput('history-input', `${PROMPT} ${input}`);

    try {
      const output = interpreter.evaluate(input);
      appendOutput('history-output-ok', output);
    } catch (error) {
      appendOutput('history-output-err', error);
    }

    element.target.value = '';
    stdoutContainer.lastChild.scrollIntoView();
  }
});

const appendOutput = (className, text) => {
  const div = document.createElement('div');
  div.classList.add(className);
  div.textContent = text;

  stdoutContainer.appendChild(div);
};

const terminal = document.getElementById('terminal');

terminal.addEventListener('click', _ => {
  inputField.focus();
});
