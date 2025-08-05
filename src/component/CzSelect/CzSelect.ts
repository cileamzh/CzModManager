export interface SelectItem {
  name: string;
  icon?: string;
  value?: any;
}

export interface ChangeEventValue {
  index: number;
  data: SelectItem;
}
