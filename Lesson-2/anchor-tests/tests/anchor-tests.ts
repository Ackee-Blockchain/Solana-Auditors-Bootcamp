import { Environment } from './environment';
import { initEnviroment } from './init-environment';
import * as programTests1 from './program-tests1';
import * as programTests2 from './program-tests2';


describe("anchor-tests", () => {
  const test_env = new Environment();
  before('Prepare', async () => {
    await initEnviroment(test_env);
  });

  describe('Program Tests 1', async () => {
    programTests1.programTests1(test_env);
  });

  describe('Program Tests 2', async () => {
    programTests2.programTests2(test_env);
  });

});
