<script setup lang="ts">
import { reactive, ref } from 'vue';
import { zodResolver } from '@primevue/forms/resolvers/zod'
import { z } from 'zod';
import { Card, Button, InputText, InputMask, Textarea, Message, Checkbox, CheckboxGroup } from 'primevue';
import { Form, type FormSubmitEvent } from '@primevue/forms';

const initialValues = reactive({
    name: '',
    company: '',
    email: '',
    tel: '',
    interests: [],
    additional: ''
});
const resolver = zodResolver(
    z.object({
        name: z.string(),
        company: z.string(),
        email: z.string(),
        tel: z.string(),
        mail: z.string(),
        interests: z.array(z.string()).min(1,"Please Select At Least One Interest"),
        additional: z.string()
    })
);
const onFormSubmit = ({ valid }: { valid: boolean }) => {
  if (valid) {
    1;
  }
};

interface NoblePayload {
  name: string;
  company: string;
  email: string;
  tel: string;
  mail: string;
  interests: Array<String>;
  additional: string;
}

async function postJson(url: string, data: NoblePayload): Promise<Response> {
  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  });

  if (!response.ok) {
    throw new Error(`Blast! Our letter was not received favorably: ${response.statusText}`);
  }
  return await response.json();
}

async function postContact(form: FormSubmitEvent): Promise<Response> {
  const payload: NoblePayload = { name: form.states.name.value, email: form.states.email.value, tel: form.states.tel.value, company: form.states.company.value, mail: form.states.mail.value, interests: form.states.interests.value, additional: form.states.additional.value, };
  return await postJson("", payload);
}
</script>
<template>
<Card>
    <template #header>Contact Form:</template>
    <template #content>
      <Form v-slot="$form" :resolver="resolver" :initialValues="initialValues" @submit.default="postContact">
        <label for="name">Name</label>
        <InputText name="name" placeholder="Name" fluid />
        <label for="company">Company</label>
        <InputText name="company" placeholder="Company" fluid />
        <label for="email">Email</label>
        <InputText id="email" placeholder="email@domain.com" fluid />
        <label for="tel">Telephone</label>
        <InputMask id="tel" mask="999-999-9999" placeholder="413-555-5555" fluid />
        <label for="mail">Address</label><br>
        <Textarea id="mail" size="large" rows="5" placeholder="Business
1 Street Address
Berkshire, MA, 01224"></Textarea>
        <div class="flex flex-col gap-2 border-2">
          <label for="interests">Interests</label>
          <CheckboxGroup name="interests">
            <Checkbox inputId="custom" value="custom" />
            <label for="custom"> Custom Molds </label>
            <Checkbox inputId="inject" value="inject" />
            <label for="inject"> Injection Molding </label>
            <Checkbox inputId="specimen" value="specimen" />
            <label for="specimen"> Specimen Molding </label>
            <Checkbox inputId="plaque" value="plaque" />
            <label for="plaque"> Plaque Molding </label>
            <Checkbox inputId="other" value="other" />
            <label for="other"> Other </label>
          </CheckboxGroup>
        </div>
        <div class="flex flex-col gap-2">
          <label for="additional">Comments Or Questions</label><br>
          <Textarea id="additional" size="large" rows="5" placeholder="Additional Details"></Textarea>
        </div>
        <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">{{ $form.name.error.message }}</Message>
        <Button type="submit" severity="secondary" label="Submit" fluid></Button>
      </Form>
    </template>
  </Card>
</template>