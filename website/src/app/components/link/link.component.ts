import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-link',
  templateUrl: './link.component.html',
  styleUrl: './link.component.scss',
})
export class LinkComponent {
  @Input({ required: true }) href!: string;
  @Input({ required: true }) name!: string;
}
