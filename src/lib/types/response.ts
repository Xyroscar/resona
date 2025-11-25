export type ResponseHeader = {
  key: string;
  value: string;
};

export type Response = {
  status: number;
  statusText: string;
  headers: ResponseHeader[];
  body: string;
  contentType: string;
  duration: number;
  size: number;
};
