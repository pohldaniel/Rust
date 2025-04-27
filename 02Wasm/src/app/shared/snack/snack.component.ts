import {Component, Inject} from '@angular/core';
import {MAT_SNACK_BAR_DATA, MatSnackBarRef} from '@angular/material/snack-bar';
import {CommonModule} from '@angular/common';

@Component({
  selector: 'app-snack',
  templateUrl: './snack.component.html',
  styleUrls: ['./snack.component.sass'],
  standalone: true,
  imports: [CommonModule]
})
export class SnackComponent {
  type: string;
  msg: string;
  errors: [string, string][];
  closeable: boolean;
  singleline: boolean;
  
  constructor(public snackBarRef: MatSnackBarRef<SnackComponent>,
              @Inject(MAT_SNACK_BAR_DATA) data: any) {   
    this.type = data.type;
    this.msg = data.msg;
    this.errors = 'errors' in data ? data.errors : '';
    this.closeable = data.closeable;
    this.singleline = data.singleline;
  }
}
