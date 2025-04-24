import {AfterViewInit, Directive} from "@angular/core";
import {loadScript} from "./tools";

type EmscriptenModuleDecorator<M extends EmscriptenModule> = (module: M) => void;
const noopModuleDecorator = (mod: EmscriptenModule) => mod;

@Directive()
export abstract class EmscriptenWasmComponent<M extends EmscriptenModule = EmscriptenModule> implements AfterViewInit {
  private resolvedModule!: M;
  protected moduleDecorator!: EmscriptenModuleDecorator<M>;

  protected constructor(private moduleExportName: string, private pathJs: string, private pathWasm: string, private pathData?: string) {}

  get module(): M {
    return this.resolvedModule;
  }

  ngAfterViewInit(): void {
    this.resolveModule();
  }

  protected resolveModule(): void {
    loadScript(this.moduleExportName, this.pathJs)
      .then(() => {
        const module = <M>{
          locateFile: (file: string) => {
            if (file.endsWith(".data")) {
              return this.pathData;
            }
            return this.pathWasm;
          }
        };
        const moduleDecorator: EmscriptenModuleDecorator<M> = this.moduleDecorator || noopModuleDecorator;
        moduleDecorator(module);

        return window[this.moduleExportName as keyof typeof window](module);
      })
      .then((mod) => {
        this.resolvedModule = mod;
      });
  }
}