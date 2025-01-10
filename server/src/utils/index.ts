export const systemIdList = ['aek', 'pek', 'sek', 'rek'];
// src/utils/index.ts
import { readdirSync, statSync } from 'fs';
import { resolve } from 'path';
import { ProjectJson } from 'src/types';
import axios from 'axios';
import { ProjectDataModel } from 'src/project/project.entity';

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

async function makeRequest(queryString: string, baseUrl: string, SESSION: string, USER_NAME: string): Promise<ProjectDataModel[]> {
    let config = {
        "method": "GET",
        "url": `${baseUrl}/rest/project?${queryString}`,
        "headers": {
            "accept": "application/json, text/plain, */*",
            "content-type": "application/x-www-form-urlencoded;charset=UTF-8",
            "Cookie": `tab-selected=0; SESSION=${SESSION}; username_1=${USER_NAME}`
        },
    }
    let res: ProjectJson = (await axios(config)).data
    return res.rows
}

function makeQueryString(date: string, systemId: string) {
    let projectNo = `${systemId.toUpperCase()}GZ${date.replace(/-/g, '')}`
    let query = {
        systemId: systemId,
        category: 'battery',
        reportType: '0',
        appraiserName: '',
        itemName: '',
        principal: '',
        startDate: date,
        endDate: date,
        projectNo: projectNo,
        page: '1',
        rows: '10000'
    }
    const queryString = new URLSearchParams(query).toString()
    return queryString
}

async function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export { makeRequest, makeQueryString, sleep }