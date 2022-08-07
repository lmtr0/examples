import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-square',
  template:`
    <button nbButton hero *ngIf="!val">{{ val }}</button>
    <button nbButton hero status="success" *ngIf="val == 'X'">{{ val }}</button>
    <button nbButton hero status="info" *ngIf="val == '0'">{{ val }}</button>
  `,
  styles:[`button{
    width:100%;
    height:100%;
    font-size: 50px;
  }`],
})
export class SquareComponent {
  @Input() val: 'X' | '0';

  constructor() { }
}
