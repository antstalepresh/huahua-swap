/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, QueryMsg, Addr, PaginatedTokensResponse, Token, TokenInfoResponse } from "./HuahuaFactory.types";
export interface HuahuaFactoryReadOnlyInterface {
  contractAddress: string;
  tokenInfo: ({
    subdenom
  }: {
    subdenom: string;
  }) => Promise<TokenInfoResponse>;
  getTokensWithPagination: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<PaginatedTokensResponse>;
}
export class HuahuaFactoryQueryClient implements HuahuaFactoryReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;
  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.tokenInfo = this.tokenInfo.bind(this);
    this.getTokensWithPagination = this.getTokensWithPagination.bind(this);
  }
  tokenInfo = async ({
    subdenom
  }: {
    subdenom: string;
  }): Promise<TokenInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      token_info: {
        subdenom
      }
    });
  };
  getTokensWithPagination = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<PaginatedTokensResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_tokens_with_pagination: {
        limit,
        start_after: startAfter
      }
    });
  };
}
export interface HuahuaFactoryInterface extends HuahuaFactoryReadOnlyInterface {
  contractAddress: string;
  sender: string;
  createToken: ({
    description,
    subdenom,
    url
  }: {
    description: string;
    subdenom: string;
    url: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  completeBondingCurve: ({
    subdenom
  }: {
    subdenom: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class HuahuaFactoryClient extends HuahuaFactoryQueryClient implements HuahuaFactoryInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;
  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createToken = this.createToken.bind(this);
    this.completeBondingCurve = this.completeBondingCurve.bind(this);
  }
  createToken = async ({
    description,
    subdenom,
    url
  }: {
    description: string;
    subdenom: string;
    url: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_token: {
        description,
        subdenom,
        url
      }
    }, fee, memo, _funds);
  };
  completeBondingCurve = async ({
    subdenom
  }: {
    subdenom: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      complete_bonding_curve: {
        subdenom
      }
    }, fee, memo, _funds);
  };
}