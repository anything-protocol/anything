import { Action, Workflow } from "./types/workflows";
import { createClient } from "./supabase/client";
import { v4 as uuidv4 } from "uuid";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getActionTemplates = async (account_id: string) => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/actions`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/actions:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching actions:', error);
    } finally {
    }
}

export const publishActionTemplate = async (account_id: string, action: Action, publish_to_team: boolean, publish_to_marketplace: boolean, publish_to_marketplace_anonymously: boolean) => {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {

            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/marketplace/action/publish`, {
                method: 'POST',
                headers: {
                    Authorization: `${session.access_token}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    publish_to_team,
                    publish_to_marketplace,
                    publish_to_marketplace_anonymously,
                    action_template_definition: action,
                }),
            });
            const data = await response.json();
            console.log('Data from /api/marketplace/action/publish:', data);
            return data;
        }
    } catch (error) {
        console.error('Error publishing action template:', error);
    }
}