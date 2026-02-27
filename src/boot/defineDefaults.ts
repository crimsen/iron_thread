import { defineBoot } from '#q-app/wrappers';
import { QInput } from 'quasar';
// "async" is optional;
// more info on params: https://v2.quasar.dev/quasar-cli-vite/boot-files
export default defineBoot(() => {
  SetComponentDefaults<QInput>(QInput, {
    outlined: true,
    dense: true,
    stackLabel: true,
  });
});
/* eslint-disable @typescript-eslint/no-explicit-any */
const SetComponentDefaults = <T>(component: any, defaults: Partial<T>): void => {
  Object.keys(defaults).forEach((prop: string) => {
    component.props[prop] =
      Array.isArray(component.props[prop]) === true || typeof component.props[prop] === 'function'
        ? {
            type: component.props[prop],
            default: (defaults as Record<string, any>)[prop],
          }
        : {
            ...component.props[prop],
            default: (defaults as Record<string, any>)[prop],
          };
  });
};
/* eslint-enable @typescript-eslint/no-explicit-any */
