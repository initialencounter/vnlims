export const systemIdList = ['aek', 'pek', 'sek', 'rek'];
// src/utils/index.ts
import { readdirSync, statSync } from 'fs';
import { resolve } from 'path';

export function getJsonFilePaths(dir: string): string[] {
    let results: string[] = [];
    const list = readdirSync(dir);

    list.forEach(file => {
        const filePath = resolve(dir, file);
        const stat = statSync(filePath);

        if (stat && stat.isDirectory()) {
            // 递归调用
            results = results.concat(getJsonFilePaths(filePath));
        } else if (file.endsWith('.json')) {
            results.push(filePath);
        }
    });

    return results;
}