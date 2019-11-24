import axios from "axios";

export default class HttpService {
    static pushHistory() { }

    static async test(): Promise<string> {
        const res = await axios.get('/api/test');
        console.log(res.data);
        return res.data;
    }
}
