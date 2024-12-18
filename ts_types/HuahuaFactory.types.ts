/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {
  bonding_curve_code_id: number;
  fee_swap_collector_address: string;
  reserve_collector_address: string;
}
export type ExecuteMsg = {
  create_token: {
    description: string;
    subdenom: string;
    url: string;
  };
} | {
  complete_bonding_curve: {
    subdenom: string;
  };
};
export type QueryMsg = {
  token_info: {
    subdenom: string;
  };
} | {
  get_tokens_with_pagination: {
    limit?: number | null;
    start_after?: string | null;
  };
};
export type Addr = string;
export interface PaginatedTokensResponse {
  tokens: Token[];
}
export interface Token {
  bonding_curve_address: Addr;
  completed: boolean;
  creator: Addr;
  denom: string;
  description: string;
  pool_id: number;
  subdenom: string;
  url: string;
}
export interface TokenInfoResponse {
  info: Token;
}