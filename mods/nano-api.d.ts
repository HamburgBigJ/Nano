type NanoElementKind = "Static" | "Powder" | "Liquid" | "Gas";
type NanoColor = [number, number, number, number];

interface NanoElementConfig {
    /** Display/internal element name. Names are case-sensitive. */
    name: string;
    /** RGBA color values from 0.0 to 1.0. */
    color: NanoColor;
    /** Movement category. Unknown runtime values become "Static". */
    kind: NanoElementKind;
    /** Name of a global behavior function, or empty/omitted for no custom behavior. */
    behavior?: string;
}

interface NanoSurroundingCell {
    x: number;
    y: number;
    id: number;
    isEmpty: boolean;
    name: string;
    kind: NanoElementKind | "";
}

type NanoSurrounding = Array<NanoSurroundingCell | null>;
type NanoBehavior = (x: number, y: number, surrounding: NanoSurrounding) => void;

declare const World: {
    /** Returns the element id at a cell. Out-of-bounds reads return 0. */
    getCell(x: number, y: number): number;
    /** Sets a cell to an element id. Out-of-bounds writes are ignored. */
    setCell(x: number, y: number, id: number): void;
    /** Returns true when the cell contains element id 0. */
    isEmpty(x: number, y: number): boolean;
    /** Swaps two cells. Out-of-bounds swaps are ignored. */
    swap(x1: number, y1: number, x2: number, y2: number): void;
    /** Returns the element name for an id, or an empty string when missing. */
    getElementName(id: number): string;
    /** Returns the element kind for an id, or an empty string when missing. */
    getElementKind(id: number): NanoElementKind | "";
};

declare const Nano: {
    /** Registers an element and returns its numeric element id. */
    registerElement(config: NanoElementConfig): number;
    /** Returns an element id by name, or -1 when missing. */
    getElementIdByName(name: string): number;
};

/** Prints a message with a [JS] prefix. */
declare function print(message: unknown): void;
