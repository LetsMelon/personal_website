import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'descriptionLink',
  standalone: true,
})
export class DescriptionLinkPipe implements PipeTransform {
  transform(
    description: string,
    descriptionLink: { text: string; href: string } | undefined
  ): string {
    if (!descriptionLink) {
      return description;
    }

    // TODO use custom link component
    const link = `<a href="${descriptionLink.href}" rel="noopener noreferrer" target="_blank">${descriptionLink.text}</a>`;
    return description.replace(descriptionLink.text, link);
  }
}
