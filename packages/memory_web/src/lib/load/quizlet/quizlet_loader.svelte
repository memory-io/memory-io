<script lang="ts">
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import { toast } from "svelte-sonner";
    async function LoadQuizlet(e:any) {
        let html = await (<HTMLInputElement>e.target).files![0].text();
        
        let parser = new DOMParser();
        
        let doc = parser.parseFromString(html, "text/html");
        let script = doc.querySelectorAll("[data-testid='set-page-card-side']");
        console.log(doc);
        let terms = [];
        //data-testid='set-page-card-side'
        let last = null;
        for (let i = 0; i < script.length; i++) {
            let side = script[i];
            let term = side.lastChild?.textContent;
            if (term) {
                if (last) {
                    terms.push({term: last, definition: term});
                    last = null;
                } else {
                    last = term;
                }
            }else{
                console.log("No term found");
            }
        }
        console.log(terms);


        
    }
    

    


    
</script>

<span>
    <Label for="fileupload">
        Import a set from quizlet.<br/>
        1. Go to the set page.<br/>
        2. Right click and save the page.<br/>
        3. Upload the saved page here.
    </Label>

    <Input type="file" name="fileupload" on:input={LoadQuizlet} accept=".html" />
</span>
