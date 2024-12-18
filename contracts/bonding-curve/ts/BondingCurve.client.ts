/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, QueryMsg, Uint128, CurveState, Coin } from "./BondingCurve.types";
export interface BondingCurveReadOnlyInterface {
  contractAddress: string;
  tokenPrice: () => Promise<Coin>;
  curveState: () => Promise<CurveState>;
}
export class BondingCurveQueryClient implements BondingCurveReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;
  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.tokenPrice = this.tokenPrice.bind(this);
    this.curveState = this.curveState.bind(this);
  }
  tokenPrice = async (): Promise<Coin> => {
    return this.client.queryContractSmart(this.contractAddress, {
      token_price: {}
    });
  };
  curveState = async (): Promise<CurveState> => {
    return this.client.queryContractSmart(this.contractAddress, {
      curve_state: {}
    });
  };
}
export interface BondingCurveInterface extends BondingCurveReadOnlyInterface {
  contractAddress: string;
  sender: string;
  buy: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  sell: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class BondingCurveClient extends BondingCurveQueryClient implements BondingCurveInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;
  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.buy = this.buy.bind(this);
    this.sell = this.sell.bind(this);
  }
  buy = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      buy: {}
    }, fee, memo, _funds);
  };
  sell = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      sell: {}
    }, fee, memo, _funds);
  };
}