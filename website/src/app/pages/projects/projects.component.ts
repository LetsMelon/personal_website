import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { ComponentsModule } from '../../components/components.module';
import { DescriptionLinkPipe } from '../home/description-link.pipe';
import { ListItemComponent } from '../home/list-item/list-item.component';

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
  selector: 'app-projects',
  standalone: true,
  imports: [
    CommonModule,
    ComponentsModule,
    ListItemComponent,
    DescriptionLinkPipe,
  ],
  templateUrl: './projects.component.html',
  styleUrl: './projects.component.scss',
})
export class ProjectsComponent {
  readonly projects: Array<Project> = [
    {
      name: 'rusvid',
      languages: [ProgrammingLanguage.rust],
      link: 'https://github.com/letsmelon/rusvid',
      description:
        "Rusvid is a Rust-based project for writing and rendering SVG animations. It uses multiple Rust crates to provide a cohesive library for animation development without requiring a GUI or CLI. Features include SVG animation rendering, a modular crate system (`rusvid_lib`, `rusvid_core`, `rusvid_effect`, `rusvid_video_encoder`), and FFmpeg integration for high-quality video output. Built with the Rust nightly compiler, it utilizes the latest features and optimizations. Rusvid aims to harness Rust's power for high-performance animation rendering.",
    },
    {
      name: 'tsql',
      languages: [ProgrammingLanguage.rust],
      link: 'https://github.com/letsmelon/tsql',
      description:
        'TSQL is an enhanced SQL tool designed for high performance and extended functionality. Built with Rust, it leverages its robust ecosystem to achieve blazingly fast execution. The project utilizes libraries like nom for efficient parsing and serde for serialization. It simplifies database operations with additional syntax sugar, making it easier to define tables, primary keys, and foreign keys. The modular architecture follows the principles of clean code and separation of concerns, ensuring maintainability and scalability.',
    },
    {
      name: 'how_many_days_until',
      languages: [ProgrammingLanguage.rust],
      link: 'https://github.com/letsmelon/how_many_days_until',
      description:
        '"How Many Days Until..." is a Rust-based command-line tool designed to calculate the number of days between two dates. It uses Rust\'s robust standard library for date manipulation and parsing. The tool offers a simple interface with options to specify the start and end dates in the format YYYY-MM-DD. By default, the start date is set to the current day. The project leverages Rust\'s performance and safety features, ensuring reliable and fast calculations. Installation is straightforward via a tar.gz archive or package managers like Homebrew for macOS. This utility is ideal for developers and users needing a quick and efficient way to calculate date differences directly from the command line.',
    },
    {
      name: 'adanui',
      languages: [ProgrammingLanguage.javascript, ProgrammingLanguage.docker],
      description:
        'Adanui Backend is a comprehensive microservices architecture built with Docker and Docker Compose, designed to anonymize DICOM files. It orchestrates several services, including a reverse proxy, API, worker processes, frontend, Redis, MongoDB, and Minio for object storage. The system manages service lifecycles with Makefile commands, supporting both production and development environments. Key features include an API for handling requests, a worker for DICOM file anonymization, and integration with Redis and MongoDB for data management. This project was my high school graduation project and a team effort in collaboration with AGFA. I was solely responsible for developing the backend infrastructure, while my colleagues focused on the frontend. The backend emphasizes scalability and maintainability, providing a solid foundation for robust web applications focused on medical data privacy.',
      descriptionLink: {
        text: 'AGFA',
        href: 'https://www.agfa.com/corporate/',
      },
    },
  ];
}
