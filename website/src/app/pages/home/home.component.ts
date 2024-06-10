import { Component } from '@angular/core';
import { ComponentsModule } from '../../components/components.module';
import { CommonModule } from '@angular/common';
import { ListItemComponent } from './list-item/list-item.component';
import { DescriptionLinkPipe } from './description-link.pipe';

export interface WorkHistory {
  name: string;
  link: string;
  startDate: Date;
  endDate?: Date;
  description?: string;
}

enum ProgrammingLanguage {
  rust = 'Rust',
  javascript = 'JavaScript',
  docker = 'Docker',
}

interface Project {
  name: string;
  languages: ProgrammingLanguage[];
  link?: string;
  description: string;
  descriptionLink?: { text: string; href: string };
}

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [
    CommonModule,
    ComponentsModule,
    ListItemComponent,
    DescriptionLinkPipe,
  ],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
})
export class HomeComponent {
  private readonly _workHistory: Array<WorkHistory> = [
    {
      name: 'Runtastic GmbH',
      link: 'https://www.runtastic.com/',
      startDate: new Date(2021, 7),
      endDate: new Date(2021, 8),
      description:
        'As an intern backend engineer, I worked on the development of microservices to meet the requirements of European data protection. This was programmed in the Ruby programming language and the web framework "Ruby on Rails".',
    },
  ];
  get workHistory(): Array<WorkHistory> {
    return this._workHistory;
  }

  readonly projects: Array<Project> = [
    {
      name: 'rusvid',
      languages: [ProgrammingLanguage.rust],
      link: 'https://github.com/letsmelon/rusvid',
      description: 'Library to create animations out of svg paths.',
    },
    {
      name: 'tsql',
      languages: [ProgrammingLanguage.rust],
      link: 'https://github.com/letsmelon/tsql',
      description: 'Custom scripting language that can be transpiled to sql.',
    },
    {
      name: 'how_many_days_until',
      languages: [ProgrammingLanguage.rust],
      link: 'https://github.com/letsmelon/how_many_days_until',
      description: 'Small cli tool to count days between two given dates.',
    },
    {
      name: 'adanui',
      languages: [ProgrammingLanguage.javascript, ProgrammingLanguage.docker],
      description:
        "My team's high school graduation project, in collaboration with AGFA, aimed to anonymize DICOM files.",
      descriptionLink: {
        text: 'AGFA',
        href: 'https://www.agfa.com/corporate/',
      },
    },
  ];

  get ageInYears(): string {
    const birthday = new Date(2003, 5, 6);
    const currentDate = new Date();

    const diff = Math.abs(birthday.getTime() - currentDate.getTime());
    const dayDiff = diff / (1000 * 60 * 60 * 24);
    const yearDiff = dayDiff / 365.25;

    return Math.floor(yearDiff).toFixed(0).toString();
  }
}
