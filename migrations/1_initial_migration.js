const Migrations = artifacts.require("Migrations");
const KOKU = artifacts.require("KOKU");

module.exports = function (deployer, network, accounts) {
  deployer.deploy(Migrations);
  const owner = accounts[0];
  console.log(owner);
  deployer.deploy(KOKU, owner);
};
