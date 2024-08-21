import { Environment } from './environment';
import { initEnviroment } from './init-environment';
import * as programTests1 from './program-tests1';

describe("anchor-tests", () => {
  const test_env = new Environment();
  before('Prepare', async () => {
    await initEnviroment(test_env);
  });

  describe('Program Tests 1', async () => {
    programTests1.programTests1(test_env);
  });

});
