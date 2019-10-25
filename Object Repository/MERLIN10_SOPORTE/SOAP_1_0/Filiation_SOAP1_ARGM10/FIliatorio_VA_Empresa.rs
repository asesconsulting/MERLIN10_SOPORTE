<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>FIliatorio_VA_Empresa</name>
   <tag></tag>
   <elementGuidId>317529b2-2eef-4d32-81c4-50b3b414db79</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.filiationonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:queryFiliation>
         &lt;!--Optional:-->
         &lt;name>
            &lt;!--Optional:-->
            &lt;birthDate> &lt;/birthDate>
            &lt;!--Optional:-->
            &lt;contributaryType> &lt;/contributaryType>
            &lt;!--Optional:-->
            &lt;documentNumber> &lt;/documentNumber>
            &lt;!--Optional:-->
            &lt;documentType> &lt;/documentType>
            &lt;!--Optional:-->
            &lt;lastName> &lt;/lastName>
            &lt;!--Optional:-->
            &lt;name> &lt;/name>
            &lt;!--Optional:-->
            &lt;sex> &lt;/sex>
            &lt;!--Optional:-->
            &lt;tributaryNumber>30676235309&lt;/tributaryNumber>
            &lt;!--Optional:-->
            &lt;tributaryType> &lt;/tributaryType>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;sector> &lt;/sector>
            &lt;!--Optional:-->
            &lt;userName> &lt;/userName>
         &lt;/name>
      &lt;/soap:queryFiliation>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>queryFiliation</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Filiatorios</defaultValue>
      <description></description>
      <id>c9672713-59fb-4dbb-b214-00afa1bee254</id>
      <masked>false</masked>
      <name>Filiatorios</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()





assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>distanciaAfip&lt;/name>&lt;value>1.00&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;customValues>&lt;name>PORTAL&lt;/name>&lt;value>PORTAL&lt;/value>&lt;/customValues>')
assertThat(response.getResponseText()).contains('&lt;birthDate>01/10/1993&lt;/birthDate>')
assertThat(response.getResponseText()).contains('&lt;birthDateFlg>AP&lt;/birthDateFlg>')
assertThat(response.getResponseText()).contains('&lt;contributorType>C&lt;/contributorType>')
assertThat(response.getResponseText()).contains('&lt;contributorTypeFlg>AP&lt;/contributorTypeFlg>')
assertThat(response.getResponseText()).contains('&lt;documentNumber>0&lt;/documentNumber>')
assertThat(response.getResponseText()).contains('&lt;documentNumberFlg>NA&lt;/documentNumberFlg>')
assertThat(response.getResponseText()).contains('&lt;documentType>&lt;/documentType>')
assertThat(response.getResponseText()).contains('&lt;documentTypeFlg>NA&lt;/documentTypeFlg>')
assertThat(response.getResponseText()).contains('&lt;lastName>&lt;/lastName>')
assertThat(response.getResponseText()).contains('&lt;lastNameFlg>NI&lt;/lastNameFlg>')
assertThat(response.getResponseText()).contains('&lt;mtnDistance>0.0&lt;/mtnDistance>')
assertThat(response.getResponseText()).contains('&lt;mtnStatus>ER&lt;/mtnStatus>')
assertThat(response.getResponseText()).contains('&lt;name>ASES CONSULTING S.A.&lt;/name>')
assertThat(response.getResponseText()).contains('&lt;nameFlg>AP&lt;/nameFlg>')
assertThat(response.getResponseText()).contains('&lt;sex>&lt;/sex>')
assertThat(response.getResponseText()).contains('&lt;sexFlg>NA&lt;/sexFlg>')
assertThat(response.getResponseText()).contains('&lt;status>OK&lt;/status>')
assertThat(response.getResponseText()).contains('&lt;tributaryNumber>30676235309&lt;/tributaryNumber>')
assertThat(response.getResponseText()).contains('&lt;tributaryNumberFlg>VA&lt;/tributaryNumberFlg>')
assertThat(response.getResponseText()).contains('&lt;tributaryType>80&lt;/tributaryType>')
assertThat(response.getResponseText()).contains('&lt;tributaryTypeFlg>AP&lt;/tributaryTypeFlg>')
</verificationScript>
   <wsdlAddress>${Filiatorios}</wsdlAddress>
</WebServiceRequestEntity>
