<template lang="pug">
.card.list-card
    .header(@click="expanded = !expanded")
        BookOpenIcon.icon(size="24")
        span History
    transition(name="expand")
        p.nothing(v-if="expanded && getData().length === 0") No history
        ul(v-else-if="expanded")
            li(v-for="d of getData()")
                p.small {{ getDateString(d.at) }}
                p.title {{ d.title }}
</template>

<script lang="ts">
import "reflect-metadata";
import { Component, Prop, Vue } from "vue-property-decorator";
import { HistoryListItem } from "../models/HistoryListItem";
import { BookOpenIcon } from "vue-feather-icons";

@Component({
    components: {
        BookOpenIcon
    }
})
export default class HistoryList extends Vue {
    private expanded: boolean = false;

    getDateString(d: Date) {
        if (d.getTime() > Date.now() - (24 * 60 * 60 * 1000)) {
            return d.toTimeString();
        } else {
            return Date.toString();
        }
    }

    getData() {
        return this.$store.state.history;
    }
}
</script>
