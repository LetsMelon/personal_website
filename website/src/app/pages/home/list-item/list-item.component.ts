import { Component, Input } from '@angular/core';
import { WorkHistory } from '../home.component';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-list-item',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './list-item.component.html',
  styleUrl: './list-item.component.scss',
})
export class ListItemComponent {
  @Input({ required: true }) workHistory!: WorkHistory;

  private formatMonth(date: Date): string {
    return date.toLocaleString('en-US', { month: 'long' });
  }

  private formatDate(date: Date): string {
    return `${this.formatMonth(date)} ${date.getFullYear()}`;
  }

  get durationText(): string {
    if (this.workHistory.endDate === undefined) {
      return `started ${this.formatDate(this.workHistory.startDate)}`;
    }

    return `${this.formatDate(this.workHistory.startDate)} - ${this.formatDate(
      this.workHistory.endDate
    )}`;
  }
}
