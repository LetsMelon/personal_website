import { Component } from '@angular/core';
import { ComponentsModule } from '../../components/components.module';
import { CommonModule } from '@angular/common';
import { ListItemComponent } from './list-item/list-item.component';
import { DescriptionLinkPipe } from './description-link.pipe';
import { RouterOutlet, RouterLink, RouterLinkActive } from '@angular/router';

export interface WorkHistory {
  name: string;
  link: string;
  startDate: Date;
  endDate?: Date;
  description?: string;
}

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [
    CommonModule,
    ComponentsModule,
    DescriptionLinkPipe,
    ListItemComponent,
    RouterLink,
    RouterLinkActive,
    RouterOutlet,
  ],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
})
export class HomeComponent {
  private readonly _workHistory: Array<WorkHistory> = [
    {
      name: 'Runtastic GmbH',
      link: 'https://www.runtastic.com/',
      startDate: new Date(2021, 6),
      endDate: new Date(2021, 7),
      description:
        'As an intern backend engineer, I worked on the development of microservices to meet the requirements of European data protection. This was programmed in the Ruby programming language and the web framework "Ruby on Rails".',
    },
    {
      name: 'celix Solutions GmbH',
      link: 'https://www.celix.at//',
      startDate: new Date(2024, 7),
    },
  ];
  get workHistory(): Array<WorkHistory> {
    return this._workHistory
      .sort((a, b) => a.startDate.getTime() - b.startDate.getTime())
      .reverse();
  }

  get ageInYears(): string {
    const birthday = new Date(2003, 5, 6);
    const currentDate = new Date();

    const diff = Math.abs(birthday.getTime() - currentDate.getTime());
    const dayDiff = diff / (1000 * 60 * 60 * 24);
    const yearDiff = dayDiff / 365.25;

    return Math.floor(yearDiff).toFixed(0).toString();
  }
}
